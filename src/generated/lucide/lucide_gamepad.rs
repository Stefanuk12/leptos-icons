use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "10" y1 = "12" y2 = "12" ></ line > < line y1 = "10" x1 = "8" x2 = "8" y2 = "14" ></ line > < line y2 = "13" x2 = "15.01" y1 = "13" x1 = "15" ></ line > < line x1 = "18" y2 = "11" x2 = "18.01" y1 = "11" ></ line > < rect y = "6" rx = "2" x = "2" width = "20" height = "12" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24")] } ;