use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y2 = "12" x1 = "6" x2 = "10" y1 = "12" ></ line > < line x1 = "8" x2 = "8" y1 = "10" y2 = "14" ></ line > < line y1 = "13" x1 = "15" x2 = "15.01" y2 = "13" ></ line > < line x1 = "18" x2 = "18.01" y1 = "11" y2 = "11" ></ line > < rect y = "6" width = "20" height = "12" x = "2" rx = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("height" , "24")] } ;