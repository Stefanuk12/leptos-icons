use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "9" x1 = "5" x2 = "19" y2 = "9" ></ line > < line x1 = "5" y2 = "15" y1 = "15" x2 = "19" ></ line > < line x2 = "5" x1 = "19" y2 = "19" y1 = "5" ></ line > < / > } } pub const LUCIDE_EQUAL_NOT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;