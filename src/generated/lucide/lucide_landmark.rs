use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "22" y1 = "22" x1 = "3" x2 = "21" ></ line > < line x1 = "6" y2 = "11" y1 = "18" x2 = "6" ></ line > < line y1 = "18" x2 = "10" x1 = "10" y2 = "11" ></ line > < line x2 = "14" y2 = "11" x1 = "14" y1 = "18" ></ line > < line y1 = "18" x2 = "18" y2 = "11" x1 = "18" ></ line > < polygon points = "12 2 20 7 4 7" ></ polygon > < / > } } pub const LucideLandmark : Path = Path { path : icon_path , props : & [("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2")] } ;