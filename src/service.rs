use std::env;
use std::ffi::OsString;

use anyhow::Result;
use clap_verbosity_flag::{Verbosity, VerbosityFilter};
use service_manager::{
    ServiceInstallCtx, ServiceLabel, ServiceManager, ServiceStartCtx, ServiceStatus,
    ServiceStatusCtx, ServiceStopCtx, ServiceUninstallCtx,
};

pub fn stop_uninstall_service(service_name: &str) -> Result<()> {
    let manager = make_service_manager()?;
    let label: ServiceLabel = service_name.parse()?;

    if manager.status(ServiceStatusCtx {
        label: label.clone(),
    })? == ServiceStatus::Running
    {
        manager.stop(ServiceStopCtx {
            label: label.clone(),
        })?;

        tracing::debug!("服务：{service_name} 停止成功");
    } else {
        tracing::debug!("服务：{service_name} 不在运行，无需停止");
    }

    if manager.status(ServiceStatusCtx {
        label: label.clone(),
    })? != ServiceStatus::NotInstalled
    {
        manager.uninstall(ServiceUninstallCtx {
            label: label.clone(),
        })?;
        tracing::debug!("服务：{service_name} 卸载成功");
    } else {
        tracing::debug!("服务：{service_name} 未安装，无需卸载");
    }

    Ok(())
}

pub fn install_start_service(service_name: &str, verbose: &Verbosity) -> Result<()> {
    stop_uninstall_service(service_name)?;

    let manager = make_service_manager()?;
    let label: ServiceLabel = service_name.parse()?;

    manager.install(ServiceInstallCtx {
        label: label.clone(),
        program: env::current_exe()?,
        args: verbose_to_str(verbose),
        contents: None,
        username: None,
        working_directory: Some(env::current_exe()?.parent().unwrap().to_path_buf()),
        environment: None,
        autostart: true,
        disable_restart_on_failure: false,
    })?;
    tracing::debug!("服务：{service_name} 安装成功");

    manager.start(ServiceStartCtx {
        label: label.clone(),
    })?;
    tracing::debug!("服务：{service_name} 启动成功");

    Ok(())
}

fn make_service_manager() -> Result<Box<dyn ServiceManager>> {
    let manager = <dyn ServiceManager>::native()?;

    if !manager.available()? {
        anyhow::bail!("服务管理不可用");
    }

    Ok(manager)
}

fn verbose_to_str(verbose: &Verbosity) -> Vec<OsString> {
    match verbose.filter() {
        VerbosityFilter::Off => vec![OsString::from("-q")],
        VerbosityFilter::Error => vec![],
        VerbosityFilter::Warn => vec![OsString::from("-v")],
        VerbosityFilter::Info => vec![OsString::from("-vv")],
        VerbosityFilter::Debug => vec![OsString::from("-vvv")],
        VerbosityFilter::Trace => vec![OsString::from("-vvvv")],
    }
}
