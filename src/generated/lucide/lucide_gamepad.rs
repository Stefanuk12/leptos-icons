use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x2 = "10" y2 = "12" x1 = "6" y1 = "12" ></ line > < line y2 = "14" x1 = "8" x2 = "8" y1 = "10" ></ line > < line x1 = "15" y1 = "13" x2 = "15.01" y2 = "13" ></ line > < line y2 = "11" y1 = "11" x2 = "18.01" x1 = "18" ></ line > < rect width = "20" rx = "2" x = "2" y = "6" height = "12" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24")] } ;