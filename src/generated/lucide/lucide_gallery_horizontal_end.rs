use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 7v10" ></ path > < path d = "M6 5v14" ></ path > < rect rx = "2" height = "18" width = "12" y = "3" x = "10" ></ rect > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round")] } ;