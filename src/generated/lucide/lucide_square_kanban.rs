use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" x = "3" height = "18" y = "3" width = "18" ></ rect > < path d = "M8 7v7" ></ path > < path d = "M12 7v4" ></ path > < path d = "M16 7v9" ></ path > < / > } } pub const LUCIDE_SQUARE_KANBAN : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;