use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < line y1 = "12" y2 = "12" x1 = "6" x2 = "10" ></ line > < line y1 = "10" y2 = "14" x1 = "8" x2 = "8" ></ line > < line x2 = "15.01" y1 = "13" x1 = "15" y2 = "13" ></ line > < line x1 = "18" y1 = "11" x2 = "18.01" y2 = "11" ></ line > < rect height = "12" rx = "2" y = "6" width = "20" x = "2" ></ rect > < / > } } pub const LUCIDE_GAMEPAD : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("height" , "24")] } ;