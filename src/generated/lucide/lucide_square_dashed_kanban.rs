use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8 7v7" ></ path > < path d = "M12 7v4" ></ path > < path d = "M16 7v9" ></ path > < path d = "M5 3a2 2 0 0 0-2 2" ></ path > < path d = "M9 3h1" ></ path > < path d = "M14 3h1" ></ path > < path d = "M19 3a2 2 0 0 1 2 2" ></ path > < path d = "M21 9v1" ></ path > < path d = "M21 14v1" ></ path > < path d = "M21 19a2 2 0 0 1-2 2" ></ path > < path d = "M14 21h1" ></ path > < path d = "M9 21h1" ></ path > < path d = "M5 21a2 2 0 0 1-2-2" ></ path > < path d = "M3 14v1" ></ path > < path d = "M3 9v1" ></ path > < / > } } pub const LUCIDE_SQUARE_DASHED_KANBAN : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;