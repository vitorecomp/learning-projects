// To parse this JSON data, do
//
//     final sprint = sprintFromJson(jsonString);

import 'package:meta/meta.dart';
import 'dart:convert';

Sprint sprintFromJson(String str) => Sprint.fromJson(json.decode(str));

String sprintToJson(Sprint data) => json.encode(data.toJson());

class Sprint {
  Sprint({
    required this.id,
    required this.nome,
    required this.link,
  });

  int id;
  String nome;
  String link;

  factory Sprint.fromJson(Map<String, dynamic> json) => Sprint(
    id: json["id"],
    nome: json["nome"],
    link: json["link"],
  );

  Map<String, dynamic> toJson() => {
    "id": id,
    "nome": nome,
    "link": link,
  };
}
