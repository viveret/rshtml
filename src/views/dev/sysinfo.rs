mvc_macro_lib::rusthtml_view_macro! {
    @viewstart "src/views/dev/_view_start.rshtml"
    @name "dev_sysinfo"
    @use sysinfo::SystemExt
    @use sysinfo::NetworkExt
    @use sysinfo::ProcessExt
    @model crate::view_models::dev::SysInfoViewModel
    @{
        view_context.insert_str("Title", "Sys Info - Dev".to_string());

        let mut sys = sysinfo::System::new_all();
        sys.refresh_all();
    }
    
    @html.link(url.url_action(false, Some(false), None, Some("index"), Some("Dev"), None, None).as_str(), "< Back to dev routes list", None)
    
    <h1>@view_context.get_str("Title")</h1>

    <h3>disks:</h3>
    <ul>
    @for disk in sys.disks() {
        <li>@format!("{:?}", disk)</li>
    }
    </ul>

    // Network interfaces name, data received and data transmitted:
    <h3>networks:</h3>
    <ul>
    @for (interface_name, data) in sys.networks() {
        <li>@format!("{}: {}/{} B", interface_name, data.received(), data.transmitted())</li>
    }
    </ul>

    // Components temperature:
    <h3>components:</h3>
    <ul>
    @for component in sys.components() {
        <li>@format!("{:?}", component)</li>
    }
    </ul>

    // RAM and swap information:
    <h3>system:</h3>
    <ul>
        <li>@format!("total memory: {} bytes", sys.total_memory())</li>
        <li>@format!("used memory : {} bytes", sys.used_memory())</li>
        <li>@format!("total swap  : {} bytes", sys.total_swap())</li>
        <li>@format!("used swap   : {} bytes", sys.used_swap())</li>

        // Display system information:
        <li>@format!("System name:             {:?}", sys.name())</li>
        <li>@format!("System kernel version:   {:?}", sys.kernel_version())</li>
        <li>@format!("System OS version:       {:?}", sys.os_version())</li>
        <li>@format!("System host name:        {:?}", sys.host_name())</li>

        // Number of CPUs:
        <li>@format!("NB CPUs: {}", sys.cpus().len())</li>
    </ul>

    // Display processes ID, name na disk usage:
    <h3>processes:</h3>
    <ul>
    @for (pid, process) in sys.processes() {
        <li>@format!("[{}] {} {:?}", pid, process.name(), process.disk_usage())</li>
    }
    </ul>
}
