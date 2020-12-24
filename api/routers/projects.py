from api.db import get_db
from asyncpg.connection import Connection
from api.db.projects import fetch_project, fetch_projects
from api.schemas.projects import ProjectOut
from typing import List, Optional
from fastapi import APIRouter, HTTPException
from fastapi.param_functions import Depends, Query

router = APIRouter(
    prefix="/projects",
    tags=["projects"],
)


@router.get("/", response_model=List[ProjectOut])
async def list_projects(semester_id: Optional[str] = Query(None), db: Connection = Depends(get_db)):
    if semester_id is not None:
        raise HTTPException(status_code=501)

    return await fetch_projects(db)


@router.get("/{project_id}", response_model=ProjectOut, responses={404: {"description": "Not found"}})
async def get_project(project_id: str, db: Connection = Depends(get_db)):
    project = await fetch_project(db, project_id)
    if project is None:
        raise HTTPException(status_code=404, detail="Project not found")
    return project
