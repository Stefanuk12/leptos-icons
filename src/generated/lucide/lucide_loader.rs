use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "12" x2 = "12" y2 = "6" y1 = "2" ></ line > < line y2 = "22" x1 = "12" y1 = "18" x2 = "12" ></ line > < line x1 = "4.93" x2 = "7.76" y2 = "7.76" y1 = "4.93" ></ line > < line x2 = "19.07" x1 = "16.24" y1 = "16.24" y2 = "19.07" ></ line > < line x1 = "2" y2 = "12" y1 = "12" x2 = "6" ></ line > < line x2 = "22" y1 = "12" y2 = "12" x1 = "18" ></ line > < line x1 = "4.93" x2 = "7.76" y2 = "16.24" y1 = "19.07" ></ line > < line x1 = "16.24" y2 = "4.93" y1 = "7.76" x2 = "19.07" ></ line > < / > } } pub const LucideLoader : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;