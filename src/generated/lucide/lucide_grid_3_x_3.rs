use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" height = "18" y = "3" rx = "2" ></ rect > < path d = "M3 9h18" ></ path > < path d = "M3 15h18" ></ path > < path d = "M9 3v18" ></ path > < path d = "M15 3v18" ></ path > < / > } } pub const LUCIDE_GRID_3_X_3 : Path = Path { path : icon_path , props : & [("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("fill" , "none")] } ;