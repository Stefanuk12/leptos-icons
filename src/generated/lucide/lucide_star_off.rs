use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8.34 8.34 2 9.27l5 4.87L5.82 21 12 17.77 18.18 21l-.59-3.43" ></ path > < path d = "M18.42 12.76 22 9.27l-6.91-1L12 2l-1.44 2.91" ></ path > < line x2 = "22" y2 = "22" y1 = "2" x1 = "2" ></ line > < / > } } pub const LUCIDE_STAR_OFF : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2")] } ;