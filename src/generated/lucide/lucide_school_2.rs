use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cy = "10" r = "1" cx = "12" ></ circle > < path d = "M22 20V8h-4l-6-4-6 4H2v12a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2Z" ></ path > < path d = "M6 17v.01" ></ path > < path d = "M6 13v.01" ></ path > < path d = "M18 17v.01" ></ path > < path d = "M18 13v.01" ></ path > < path d = "M14 22v-5a2 2 0 0 0-2-2v0a2 2 0 0 0-2 2v5" ></ path > < / > } } pub const LucideSchool2 : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24")] } ;