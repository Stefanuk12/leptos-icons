use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" y = "3" height = "18" width = "18" rx = "2" ></ rect > < path d = "M8 7v7" ></ path > < path d = "M12 7v4" ></ path > < path d = "M16 7v9" ></ path > < / > } } pub const LUCIDE_SQUARE_KANBAN : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round")] } ;