use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "5" r = "3" cx = "18" ></ circle > < circle cy = "12" cx = "6" r = "3" ></ circle > < circle r = "3" cy = "19" cx = "18" ></ circle > < line x1 = "8.59" y2 = "17.49" y1 = "13.51" x2 = "15.42" ></ line > < line x2 = "8.59" y2 = "10.49" y1 = "6.51" x1 = "15.41" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("fill" , "none")] } ;