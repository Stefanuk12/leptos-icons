use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" height = "18" x = "3" width = "18" rx = "2" ></ rect > < path d = "M7.5 3v18" ></ path > < path d = "M12 3v18" ></ path > < path d = "M16.5 3v18" ></ path > < / > } } pub const LUCIDE_COLUMNS_4 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24")] } ;