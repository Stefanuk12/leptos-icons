use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" y = "3" height = "18" rx = "2" ></ rect > < path d = "M7.5 3v18" ></ path > < path d = "M12 3v18" ></ path > < path d = "M16.5 3v18" ></ path > < / > } } pub const LUCIDE_COLUMNS_4 : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;