use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" height = "18" y = "3" x = "3" width = "18" ></ rect > < path d = "M12 3v18" ></ path > < / > } } pub const LUCIDE_COLUMNS_2 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;