use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" height = "18" width = "18" y = "3" ></ rect > < path d = "M3 12h18" ></ path > < path d = "M12 3v18" ></ path > < / > } } pub const LUCIDE_GRID_2_X_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24")] } ;