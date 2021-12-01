FROM mcr.microsoft.com/dotnet/runtime:6.0 AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/sdk:6.0 AS build
WORKDIR /src
COPY ["day1/day1.csproj", "day1/"]
RUN dotnet restore "day1/day1.csproj"
COPY . .
WORKDIR "/src/day1"
RUN dotnet build "day1.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "day1.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "day1.dll"]
