use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" width = "18" rx = "2" height = "18" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M3 15h18" ></ path > < path d = "M9 3v18" ></ path > < path d = "M15 3v18" ></ path > < / > } } pub const LUCIDE_GRID_3_X_3 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;