use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "17" cy = "4" r = "2" ></ circle > < path d = "M15.59 5.41 5.41 15.59" ></ path > < circle cy = "17" cx = "4" r = "2" ></ circle > < path d = "M12 22s-4-9-1.5-11.5S22 12 22 12" ></ path > < / > } } pub const LUCIDE_TANGENT : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;