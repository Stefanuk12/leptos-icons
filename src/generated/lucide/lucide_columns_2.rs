use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" y = "3" rx = "2" height = "18" ></ rect > < path d = "M12 3v18" ></ path > < / > } } pub const LUCIDE_COLUMNS_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;