#[cfg(test)]
mod tests {
    use crate::scenarios::ssh_forwarding_run::SshForwardingRun;

    #[test]
    fn composed_application_mounts_the_host_agent_socket_in_the_job_container() {
        let run = SshForwardingRun::execute();

        assert_eq!(run.outcome(), &Ok(()));
        assert!(run.created_agent_mount());
        assert!(run.created_agent_environment());
    }
}
