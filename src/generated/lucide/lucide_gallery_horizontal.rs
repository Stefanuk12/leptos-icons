use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3v18" ></ path > < rect height = "18" x = "6" width = "12" rx = "2" y = "3" ></ rect > < path d = "M22 3v18" ></ path > < / > } } pub const LUCIDE_GALLERY_HORIZONTAL : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;