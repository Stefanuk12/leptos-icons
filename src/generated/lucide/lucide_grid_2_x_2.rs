use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" x = "3" height = "18" width = "18" rx = "2" ></ rect > < path d = "M3 12h18" ></ path > < path d = "M12 3v18" ></ path > < / > } } pub const LUCIDE_GRID_2_X_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;