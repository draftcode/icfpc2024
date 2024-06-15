# Pythonバックエンドについて

FastAPIというJSONを適当に返すためのフレームワークと、SQLModelというORMを使って
います。

* FastAPI: https://fastapi.tiangolo.com/
* SQLModel: https://sqlmodel.tiangolo.com/

## ローカルでの実行

一回だけ `docker compose up -d`
をしてください。DBがバックグラウンドで立ち上がります。コンテスト期間中は立ち上げ
っぱなしで良いと思います。

一回だけ `rye sync` をしてください。適当にライブラリなどをvirtualenvに
インストールします。

あとは `rye run dev-server` でサーバーが立ち上がります。`http://localhost:8000/`
にアクセスすると自動生成APIドキュメントが見れます。

## FastAPIについて

多分見たまんまの使い方をすればいいと思います。`api.py`を見ればわかると思います。
JSONになりそうなオブジェクト(dictとか)を返すだけです。


## データベースについて

かいつまんで説明しますが、詳しくは https://sqlmodel.tiangolo.com/
を読むと良いと思います。中身はSQLAlchemyとAlembicです。

### テーブル定義をしたい

大まかな流れとしては

1. PythonのクラスでDBのテーブル定義
2. `rye run db-migrate` でDBのそのクラス定義から自動でマイグレーションファイルを
   生成。`db_migrations/versions/` にファイルが生成される
3. `rye run db-upgrade` でDBにマイグレーションを適用

あとは適当にデプロイ時に`rye run db-upgrade`を実行すればOKです。

適当にPythonのクラスをコピペで作ってください。一回やればわかります。詳しくは

https://sqlmodel.tiangolo.com/tutorial/create-db-and-table/#define-the-fields-columns

### SELECT

HTTPリクエストハンドラ内であれば、`session`というやつがあると思うので、それを使って
SELECTクエリをかけます。Autocompleteが効くので、適当に使ってください。

```python
>>> problems: list[Problem] = session.exec(
        select(Problem).where(Problem.name == "test")
    ).all())
```

https://sqlmodel.tiangolo.com/tutorial/where/

### INSERT

Tableを定義したクラスは普通にPythonオブジェクトとしてインスタンス化できるので、
それを新しく作って`session.add()`して`session.commit()`すればOKです。

https://sqlmodel.tiangolo.com/tutorial/insert/#commit-the-session-changes

### UPDATE

SELECTして返ってきたオブジェクトを適当にいじって、`session.commit()`すればOKです。

https://sqlmodel.tiangolo.com/tutorial/update/#commit-the-session

## Typecheck / Autocompletion

AutocompletionはVSCodeを使っていれば多分動くんですが、そうではない場合はPyright
かJediで動きます。より現代的で速いのはPyrightではあります。

Typecheckは伝統的にはmypyだったんですが、最近はPyrightが良いような感じがします。

