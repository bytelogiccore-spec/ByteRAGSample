use byterag_codegraph::parser::parse_file;
use byterag_codegraph::types::{EdgeType, NodeType};

#[test]
fn test_parse_rust_code() {
    let code = r#"
        pub struct UserSession {
            id: String,
        }

        pub trait Authenticatable {
            fn authenticate(&self) -> bool;
        }

        impl Authenticatable for UserSession {
            fn authenticate(&self) -> bool {
                true
            }
        }

        pub fn login(session: &UserSession) {
            session.authenticate();
        }
    "#;

    let (nodes, edges) = parse_file("src/auth.rs", code);
    assert!(!nodes.is_empty(), "Nodes should not be empty");
    assert!(!edges.is_empty(), "Edges should not be empty");

    let has_struct = nodes.iter().any(|n| n.name == "UserSession" && n.node_type == NodeType::Struct);
    let has_trait = nodes.iter().any(|n| n.name == "Authenticatable" && n.node_type == NodeType::Trait);
    let has_fn = nodes.iter().any(|n| n.name == "login" && n.node_type == NodeType::Function);

    assert!(has_struct, "Must parse UserSession struct");
    assert!(has_trait, "Must parse Authenticatable trait");
    assert!(has_fn, "Must parse login function");

    let has_impl_edge = edges.iter().any(|e| e.edge_type == EdgeType::Implements);
    assert!(has_impl_edge, "Must detect Implements edge");
}

#[test]
fn test_parse_cpp_code() {
    let code = r#"
        #include <vector>
        #include "engine/core.hpp"

        class BaseRenderer {
        };

        class VulkanRenderer : public BaseRenderer {
        };
    "#;

    let (nodes, edges) = parse_file("src/renderer.cpp", code);
    let has_class = nodes.iter().any(|n| n.name == "VulkanRenderer" && n.node_type == NodeType::Class);
    let has_include = edges.iter().any(|e| e.edge_type == EdgeType::Includes);
    let has_extends = edges.iter().any(|e| e.edge_type == EdgeType::Extends);

    assert!(has_class, "Must parse C++ VulkanRenderer class");
    assert!(has_include, "Must parse include directives");
    assert!(has_extends, "Must parse inheritance extends edge");
}

#[test]
fn test_parse_csharp_code() {
    let code = r#"
        using System;
        using ByteLogic.Core;

        public interface IRepository {
        }

        public class SqlRepository : IRepository {
        }
    "#;

    let (nodes, edges) = parse_file("Services/SqlRepository.cs", code);
    let has_iface = nodes.iter().any(|n| n.name == "IRepository" && n.node_type == NodeType::Interface);
    let has_class = nodes.iter().any(|n| n.name == "SqlRepository" && n.node_type == NodeType::Class);
    let has_using = edges.iter().any(|e| e.edge_type == EdgeType::Using);
    let has_impl = edges.iter().any(|e| e.edge_type == EdgeType::Implements);

    assert!(has_iface, "Must parse C# interface");
    assert!(has_class, "Must parse C# class");
    assert!(has_using, "Must parse using namespaces");
    assert!(has_impl, "Must parse implements edge");
}

#[test]
fn test_parse_typescript_code() {
    let code = r#"
        import { HttpClient } from './http';

        export interface UserConfig {
            theme: string;
        }

        export class ApiClient extends HttpClient implements UserConfig {
            fetchData() {}
        }
    "#;

    let (nodes, edges) = parse_file("src/api.ts", code);
    let has_iface = nodes.iter().any(|n| n.name == "UserConfig" && n.node_type == NodeType::Interface);
    let has_class = nodes.iter().any(|n| n.name == "ApiClient" && n.node_type == NodeType::Class);
    let has_import = edges.iter().any(|e| e.edge_type == EdgeType::Imports);
    let has_ext = edges.iter().any(|e| e.edge_type == EdgeType::Extends);
    let has_impl = edges.iter().any(|e| e.edge_type == EdgeType::Implements);

    assert!(has_iface, "Must parse TS interface");
    assert!(has_class, "Must parse TS class");
    assert!(has_import, "Must parse import edge");
    assert!(has_ext, "Must parse extends edge");
    assert!(has_impl, "Must parse implements edge");
}

#[test]
fn test_parse_python_code() {
    let code = r#"
        from byterag.core import Database, CsrGraph
        import os, sys

        class ModelRunner:
            def run_model(self):
                pass
    "#;

    let (nodes, edges) = parse_file("scripts/run.py", code);
    let has_class = nodes.iter().any(|n| n.name == "ModelRunner" && n.node_type == NodeType::Class);
    let has_fn = nodes.iter().any(|n| n.name == "run_model" && n.node_type == NodeType::Function);
    let has_import = edges.iter().any(|e| e.edge_type == EdgeType::Imports);

    assert!(has_class, "Must parse Python class");
    assert!(has_fn, "Must parse Python def");
    assert!(has_import, "Must parse Python import");
}
