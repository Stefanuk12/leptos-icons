use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M9.26 9.26 3 11v3l14.14 3.14" ></ path > < path d = "M21 15.34V6l-7.31 2.03" ></ path > < path d = "M11.6 16.8a3 3 0 1 1-5.8-1.6" ></ path > < line y2 = "22" y1 = "2" x1 = "2" x2 = "22" ></ line > < / > } } pub const LucideMegaphoneOff : Path = Path { path : icon_path , props : & [("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;