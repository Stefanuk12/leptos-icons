use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "16" rx = "2" width = "20" x = "2" y = "4" ></ rect > < path d = "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" ></ path > < / > } } pub const LUCIDE_MAIL : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor")] } ;