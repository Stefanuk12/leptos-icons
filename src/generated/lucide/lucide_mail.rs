use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "2" rx = "2" height = "16" y = "4" width = "20" ></ rect > < path d = "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" ></ path > < / > } } pub const LUCIDE_MAIL : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("height" , "24") , ("width" , "24") , ("stroke" , "currentColor")] } ;