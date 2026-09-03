use assert_cmd::Command;

pub fn pc() -> Command {
    let mut cmd = Command::cargo_bin("pc").unwrap();
    // 隔离宿主环境，避免开发者本机的认证信息影响断言
    cmd.env_remove("PC_TOKEN")
        .env_remove("PC_CLIENT_ID")
        .env_remove("PC_CLIENT_SECRET")
        .env_remove("PC_OPEN_API_BASE_URL");
    // 切换到不含 .env 的目录，避免仓库根目录的 .env 被自动加载污染断言
    cmd.current_dir(std::env::temp_dir());
    cmd
}
