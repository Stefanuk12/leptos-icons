use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "2" x1 = "12" x2 = "12" y2 = "6" ></ line > < line y2 = "22" y1 = "18" x1 = "12" x2 = "12" ></ line > < line x2 = "7.76" y1 = "4.93" y2 = "7.76" x1 = "4.93" ></ line > < line y2 = "19.07" x1 = "16.24" x2 = "19.07" y1 = "16.24" ></ line > < line y1 = "12" x1 = "2" y2 = "12" x2 = "6" ></ line > < line y1 = "12" x1 = "18" y2 = "12" x2 = "22" ></ line > < line x2 = "7.76" x1 = "4.93" y1 = "19.07" y2 = "16.24" ></ line > < line y1 = "7.76" y2 = "4.93" x2 = "19.07" x1 = "16.24" ></ line > < / > } } pub const LUCIDE_LOADER : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;