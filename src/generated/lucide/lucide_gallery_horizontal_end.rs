use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 7v10" ></ path > < path d = "M6 5v14" ></ path > < rect rx = "2" width = "12" height = "18" x = "10" y = "3" ></ rect > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL_END : Path = Path { path : icon_path , props : & [("height" , "24") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;