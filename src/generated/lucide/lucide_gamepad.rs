use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line x1 = "6" x2 = "10" y2 = "12" y1 = "12" ></ line > < line x1 = "8" x2 = "8" y2 = "14" y1 = "10" ></ line > < line x1 = "15" y1 = "13" y2 = "13" x2 = "15.01" ></ line > < line y1 = "11" x1 = "18" x2 = "18.01" y2 = "11" ></ line > < rect rx = "2" width = "20" height = "12" x = "2" y = "6" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none")] } ;