use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3v18" ></ path > < rect width = "12" y = "3" rx = "2" x = "6" height = "18" ></ rect > < path d = "M22 3v18" ></ path > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor")] } ;