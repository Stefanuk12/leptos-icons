use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 5v11" ></ path > < path d = "M12 5v6" ></ path > < path d = "M18 5v14" ></ path > < / > } } pub const LUCIDE_KANBAN : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;