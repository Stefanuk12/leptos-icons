use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M15.59 5.41 5.41 15.59" ></ path > < circle cy = "17" r = "2" cx = "4" ></ circle > < path d = "M12 22s-4-9-1.5-11.5S22 12 22 12" ></ path > < / > } } pub const LucideTangent : Path = Path { path : icon_path , props : & [("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;