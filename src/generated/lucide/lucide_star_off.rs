use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M8.34 8.34 2 9.27l5 4.87L5.82 21 12 17.77 18.18 21l-.59-3.43" ></ path > < path d = "M18.42 12.76 22 9.27l-6.91-1L12 2l-1.44 2.91" ></ path > < line y2 = "22" x1 = "2" y1 = "2" x2 = "22" ></ line > < / > } } pub const LUCIDE_STAR_OFF : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("fill" , "none") , ("width" , "24")] } ;