use libsql::Connection;

use crate::models::{Bookmark, Group, Tag, Workspace};
use crate::repository::{PortHubRepository, RepositoryResult};

pub struct LibSqlRepository {
    connection: Connection,
}

impl LibSqlRepository {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }
}

impl PortHubRepository for LibSqlRepository {
    fn add_workspace(&self, workspace: &Workspace) -> RepositoryResult<()> {
        todo!()
    }

    fn get_workspaces(&self) -> RepositoryResult<Vec<Workspace>> {
        todo!()
    }

    fn get_workspace_by_id(&self, id: &str) -> RepositoryResult<Workspace> {
        todo!()
    }

    fn update_workspace(&self, workspace: &Workspace) -> RepositoryResult<()> {
        todo!()
    }

    fn delete_workspace(&self, workspace_id: &str) -> RepositoryResult<()> {
        todo!()
    }

    fn add_group(&self, workspace_id: &str, group: &Group) -> RepositoryResult<()> {
        todo!()
    }

    fn get_groups_by_workspace(&self, workspace_id: &str) -> RepositoryResult<Vec<Group>> {
        todo!()
    }

    fn get_group_by_id(&self, id: &str) -> RepositoryResult<Group> {
        todo!()
    }

    fn update_group(&self, workspace_id: &str, group: &Group) -> RepositoryResult<()> {
        todo!()
    }

    fn delete_group(&self, id: &str) -> RepositoryResult<()> {
        todo!()
    }

    fn add_bookmark(&self, group_id: &str, bookmark: &Bookmark) -> RepositoryResult<()> {
        todo!()
    }

    fn get_bookmarks_by_group(&self, group_id: &str) -> RepositoryResult<Vec<Bookmark>> {
        todo!()
    }

    fn get_bookmark_by_id(&self, id: &str) -> RepositoryResult<Bookmark> {
        todo!()
    }

    fn update_bookmark(&self, bookmark: &Bookmark) -> RepositoryResult<()> {
        todo!()
    }

    fn delete_bookmark(&self, id: &str) -> RepositoryResult<()> {
        todo!()
    }

    fn add_tag(&self, workspace_id: &str, name: &str, color: &str) -> RepositoryResult<Tag> {
        todo!()
    }

    fn get_tags_by_workspace(&self, workspace_id: &str) -> RepositoryResult<Vec<Tag>> {
        todo!()
    }

    fn get_tags_by_group(&self, group_id: &str) -> RepositoryResult<Vec<Tag>> {
        todo!()
    }

    fn add_tag_to_bookmark(&self, bookmark_id: &str, tag_id: &str) -> RepositoryResult<()> {
        todo!()
    }

    fn remove_tag_from_bookmark(&self, bookmark_id: &str, tag_id: &str) -> RepositoryResult<()> {
        todo!()
    }

    fn find_bookmarks(&self, query: &str) -> RepositoryResult<Vec<Bookmark>> {
        todo!()
    }

    fn get_favorite_bookmarks(&self) -> RepositoryResult<Vec<Bookmark>> {
        todo!()
    }
}
