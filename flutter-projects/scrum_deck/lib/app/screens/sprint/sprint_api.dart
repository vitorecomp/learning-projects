import 'dart:convert';
import 'dart:developer';

import 'package:http/http.dart';
import 'package:scrum_deck/shared/models/sprint.dart';
import 'package:scrum_deck/shared/util/contants.dart';

class SprintApi {
  static const PATH = 'sprint';
  final Client _client;

  SprintApi(this._client);

  Future<List<Sprint>> fetchSprints() async {
    final response =
        await _client.get(Uri.parse('${Constants.API_BASE_URL}/$PATH'));
    if (response.statusCode >= 200 && response.statusCode < 300) {
      final List<dynamic> jSprints = json.decode(response.body);
      final sprints = jSprints.map((js) => Sprint.fromJson(js)).toList();
      return sprints;
    }

    print("Error ao buscar sprints");
    print(response.statusCode);
    print(response.body);
    throw Exception("Falha ao buscar sprints");
  }

  Future<Sprint> getSprint() async {
    throw UnimplementedError();
  }

  deleteSprint(id) async {
    final response =
        await _client.delete(Uri.parse('${Constants.API_BASE_URL}/$PATH/$id'));
    if (response.statusCode >= 200 && response.statusCode < 300) {
      return;
    }

    print("Error ao deletar sprints");
    print(response.statusCode);
    print(response.body);
    throw Exception("Falha ao deletar sprints");
  }

  addSprint(Sprint sprint) async {
    sprint.toJson();
    final response = await _client.post(
        Uri.parse('${Constants.API_BASE_URL}/$PATH'),
        headers: {"Content-Type": "application/json"},
        body: sprint.toJson());
    if (response.statusCode >= 200 && response.statusCode < 300) {
      return;
    }

    print("Error ao salvar sprints");
    print(response.statusCode);
    print(response.body);
    throw Exception("Falha ao salvar sprints");
  }
}
