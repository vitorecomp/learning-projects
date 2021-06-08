import 'package:bloc_pattern/bloc_pattern.dart';
import 'package:flutter/cupertino.dart';
import 'package:rxdart/rxdart.dart';
import 'package:scrum_deck/app/screens/sprint/sprint_api.dart';
import 'package:scrum_deck/shared/models/sprint.dart';

class SprintBloc extends BlocBase {
  final SprintApi _api;

  SprintBloc(this._api);

  var _sprints = <Sprint>[];

  late final _sprintFetcher = PublishSubject<List<Sprint>>();
  late final _loading = BehaviorSubject<bool>();

  final formKey = GlobalKey<FormState>();

  final nameController = TextEditingController();
  final linkController = TextEditingController();

  Stream<List<Sprint>> get sprints => _sprintFetcher.stream;
  Stream<bool> get loading => _loading.stream;

  doFetch() async {
    _loading.sink.add(true);
    final sprints = await _api.fetchSprints();
    _sprints = sprints;
    _sprintFetcher.sink.add(sprints);
    _loading.sink.add(false);
  }

  doDelete(id) async {
    _loading.sink.add(true);
    await _api.deleteSprint(id);
    _sprints.removeWhere((sp) => sp.id == id);
    _sprintFetcher.sink.add(_sprints);
    _loading.sink.add(false);
  }

  void doSave() async {
    final sprint = new Sprint(
      nome:nameController.text,
      link:linkController.text,
    );
    nameController.clear();
    linkController.clear();

    await _api.addSprint(sprint);
    doFetch();
  }

  @override
  void dispose(){
    _sprintFetcher.close();
    _loading.close();
    super.dispose();
  }

}