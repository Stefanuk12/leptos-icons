use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9.26 9.26 3 11v3l14.14 3.14" ></ path > < path d = "M21 15.34V6l-7.31 2.03" ></ path > < path d = "M11.6 16.8a3 3 0 1 1-5.8-1.6" ></ path > < line x1 = "2" x2 = "22" y1 = "2" y2 = "22" ></ line > < / > } } pub const LUCIDE_MEGAPHONE_OFF : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;