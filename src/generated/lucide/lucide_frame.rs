use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "22" y1 = "6" y2 = "6" x2 = "2" ></ line > < line x2 = "2" y1 = "18" y2 = "18" x1 = "22" ></ line > < line y1 = "2" y2 = "22" x2 = "6" x1 = "6" ></ line > < line y1 = "2" x1 = "18" x2 = "18" y2 = "22" ></ line > < / > } } pub const LucideFrame : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24")] } ;