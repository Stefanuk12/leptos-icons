use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" y2 = "4" x1 = "19" y1 = "4" ></ line > < line y1 = "20" x2 = "5" y2 = "20" x1 = "14" ></ line > < line x1 = "15" x2 = "9" y1 = "4" y2 = "20" ></ line > < / > } } pub const LUCIDE_ITALIC : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("width" , "24") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;