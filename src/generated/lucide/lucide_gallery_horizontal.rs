use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3v18" ></ path > < rect width = "12" x = "6" y = "3" rx = "2" height = "18" ></ rect > < path d = "M22 3v18" ></ path > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;