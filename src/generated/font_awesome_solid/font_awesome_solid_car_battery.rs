use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M80 96c0-17.7 14.3-32 32-32h64c17.7 0 32 14.3 32 32l96 0c0-17.7 14.3-32 32-32h64c17.7 0 32 14.3 32 32h16c35.3 0 64 28.7 64 64V384c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V160c0-35.3 28.7-64 64-64l16 0zm304 96c0-8.8-7.2-16-16-16s-16 7.2-16 16v32H320c-8.8 0-16 7.2-16 16s7.2 16 16 16h32v32c0 8.8 7.2 16 16 16s16-7.2 16-16V256h32c8.8 0 16-7.2 16-16s-7.2-16-16-16H384V192zM80 240c0 8.8 7.2 16 16 16h96c8.8 0 16-7.2 16-16s-7.2-16-16-16H96c-8.8 0-16 7.2-16 16z" ></ path > < / > } } pub const FontAwesomeSolidCarBattery : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 512 512")] } ;