use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "19" y2 = "4" x2 = "10" y1 = "4" ></ line > < line y1 = "20" x1 = "14" x2 = "5" y2 = "20" ></ line > < line y1 = "4" y2 = "20" x1 = "15" x2 = "9" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2")] } ;