use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M22 13V6a2 2 0 0 0-2-2H4a2 2 0 0 0-2 2v12c0 1.1.9 2 2 2h8" ></ path > < path d = "m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7" ></ path > < path d = "M19 16v6" ></ path > < path d = "M16 19h6" ></ path > < / > } } pub const LUCIDE_MAIL_PLUS : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;