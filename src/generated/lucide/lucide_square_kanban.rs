use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" height = "18" y = "3" x = "3" ></ rect > < path d = "M8 7v7" ></ path > < path d = "M12 7v4" ></ path > < path d = "M16 7v9" ></ path > < / > } } pub const LUCIDE_SQUARE_KANBAN : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;