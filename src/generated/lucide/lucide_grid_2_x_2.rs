use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" height = "18" y = "3" rx = "2" ></ rect > < path d = "M3 12h18" ></ path > < path d = "M12 3v18" ></ path > < / > } } pub const LUCIDE_GRID_2_X_2 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24")] } ;