use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle r = "3" cx = "18" cy = "5" ></ circle > < circle cy = "12" r = "3" cx = "6" ></ circle > < circle r = "3" cx = "18" cy = "19" ></ circle > < line x1 = "8.59" y2 = "17.49" x2 = "15.42" y1 = "13.51" ></ line > < line y1 = "6.51" y2 = "10.49" x1 = "15.41" x2 = "8.59" ></ line > < / > } } pub const LUCIDE_SHARE_2 : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke-linecap" , "round")] } ;