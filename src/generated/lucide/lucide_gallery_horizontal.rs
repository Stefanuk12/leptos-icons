use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3v18" ></ path > < rect x = "6" width = "12" height = "18" y = "3" rx = "2" ></ rect > < path d = "M22 3v18" ></ path > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;